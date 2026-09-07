use super::{Error, Result};

#[derive(Clone, Copy)]
pub(super) struct Fdt<'a> {
    structure: &'a [u8],
    strings: &'a [u8],
}

#[derive(Clone, Copy)]
pub(super) struct Node<'a> {
    tree: Fdt<'a>,
    pub name: &'a str,
    start: usize,
}

struct Cursor<'a> {
    tree: Fdt<'a>,
    position: usize,
}

enum Token<'a> {
    Begin(&'a str),
    Property(&'a str, &'a [u8]),
    End,
    Finish,
    Nop,
}

pub(super) fn word(bytes: &[u8]) -> Result<u32> {
    Ok(u32::from_be_bytes(
        bytes.try_into().map_err(|_| Error::Format)?,
    ))
}

pub(super) fn text(bytes: &[u8]) -> Result<&str> {
    let bytes = bytes.strip_suffix(&[0]).ok_or(Error::Format)?;
    if bytes.contains(&0) {
        return Err(Error::Unsupported);
    }
    core::str::from_utf8(bytes).map_err(|_| Error::Format)
}

pub(super) fn integer(bytes: &[u8]) -> Result<usize> {
    let value = match bytes.len() {
        4 => u64::from(word(bytes)?),
        8 => u64::from_be_bytes(bytes.try_into().map_err(|_| Error::Format)?),
        _ => return Err(Error::Format),
    };
    usize::try_from(value).map_err(|_| Error::Bounds)
}

fn range(bytes: &[u8], start: usize, size: usize) -> Result<&[u8]> {
    bytes
        .get(start..start.checked_add(size).ok_or(Error::Bounds)?)
        .ok_or(Error::Bounds)
}

fn align(value: usize) -> Result<usize> {
    Ok(value.checked_add(3).ok_or(Error::Bounds)? & !3)
}

impl<'a> Fdt<'a> {
    pub fn size(header: &[u8]) -> Result<usize> {
        if word(range(header, 0, 4)?)? != 0xd00d_feed {
            return Err(Error::Format);
        }
        let size = word(range(header, 4, 4)?)? as usize;
        if size < 40 {
            return Err(Error::Format);
        }
        Ok(size)
    }

    pub fn root(bytes: &'a [u8]) -> Result<Node<'a>> {
        let total = Self::size(bytes)?;
        let bytes = range(bytes, 0, total)?;
        let field = |offset| word(range(bytes, offset, 4)?).map(|v| v as usize);
        if field(20)? != 17 || field(24)? > 17 {
            return Err(Error::Unsupported);
        }
        let structure = field(8)?;
        let strings = field(12)?;
        let reserve = field(16)?;
        let struct_size = field(36)?;
        let string_size = field(32)?;
        let tree = Self {
            structure: range(bytes, structure, struct_size)?,
            strings: range(bytes, strings, string_size)?,
        };
        if structure < 40
            || strings < 40
            || reserve < 40
            || reserve % 8 != 0
            || structure % 4 != 0
            || struct_size % 4 != 0
            || (structure < strings + string_size && strings < structure + struct_size)
        {
            return Err(Error::Format);
        }
        let mut end = reserve;
        loop {
            let entry = range(bytes, end, 16)?;
            end += 16;
            if entry.iter().all(|&b| b == 0) {
                break;
            }
        }
        if (reserve < structure + struct_size && structure < end)
            || (reserve < strings + string_size && strings < end)
        {
            return Err(Error::Format);
        }
        let mut cursor = Cursor { tree, position: 0 };
        if !matches!(cursor.next()?, Token::Begin("")) {
            return Err(Error::Format);
        }
        let root = Node {
            tree,
            name: "",
            start: cursor.position,
        };
        let mut depth = 1usize;
        // Bounded nesting; properties must precede a node's children.
        let mut children = [false; 32];
        loop {
            match cursor.next()? {
                Token::Begin(name) => {
                    if depth == 0
                        || depth == children.len()
                        || name.is_empty()
                        || name.contains('/')
                    {
                        return Err(Error::Format);
                    }
                    children[depth - 1] = true;
                    children[depth] = false;
                    depth += 1;
                }
                Token::Property(name, _) => {
                    if depth == 0 || children[depth - 1] || name.is_empty() {
                        return Err(Error::Format);
                    }
                }
                Token::End => depth = depth.checked_sub(1).ok_or(Error::Format)?,
                Token::Finish if depth == 0 => break,
                Token::Finish => return Err(Error::Format),
                Token::Nop => {}
            }
        }
        if tree.structure[cursor.position..].iter().any(|&b| b != 0) {
            return Err(Error::Format);
        }
        Ok(root)
    }
}

impl<'a> Cursor<'a> {
    fn next(&mut self) -> Result<Token<'a>> {
        let token = word(range(self.tree.structure, self.position, 4)?)?;
        self.position += 4;
        Ok(match token {
            1 => {
                let rest = self
                    .tree
                    .structure
                    .get(self.position..)
                    .ok_or(Error::Bounds)?;
                let length = rest.iter().position(|&b| b == 0).ok_or(Error::Format)?;
                let name = text(&rest[..=length])?;
                self.position = align(self.position + length + 1)?;
                Token::Begin(name)
            }
            2 => Token::End,
            3 => {
                let length = word(range(self.tree.structure, self.position, 4)?)? as usize;
                let offset = word(range(self.tree.structure, self.position + 4, 4)?)? as usize;
                self.position += 8;
                let names = self.tree.strings.get(offset..).ok_or(Error::Bounds)?;
                let end = names.iter().position(|&b| b == 0).ok_or(Error::Format)?;
                let name = text(&names[..=end])?;
                let value = range(self.tree.structure, self.position, length)?;
                self.position = align(self.position.checked_add(length).ok_or(Error::Bounds)?)?;
                Token::Property(name, value)
            }
            4 => Token::Nop,
            9 => Token::Finish,
            _ => return Err(Error::Format),
        })
    }
}

impl<'a> Node<'a> {
    pub fn property(self, name: &str) -> Result<Option<&'a [u8]>> {
        let mut cursor = Cursor {
            tree: self.tree,
            position: self.start,
        };
        let mut value = None;
        loop {
            match cursor.next()? {
                Token::Property(key, bytes) if key == name => {
                    if value.replace(bytes).is_some() {
                        return Err(Error::Format);
                    }
                }
                Token::Property(_, _) | Token::Nop => {}
                _ => return Ok(value),
            }
        }
    }

    pub fn required(self, name: &str) -> Result<&'a [u8]> {
        self.property(name)?.ok_or(Error::Missing)
    }

    pub fn children(self) -> Children<'a> {
        Children {
            cursor: Cursor {
                tree: self.tree,
                position: self.start,
            },
            done: false,
        }
    }

    pub fn child(self, name: &str) -> Result<Self> {
        let mut result = None;
        for child in self.children() {
            let child = child?;
            if child.name == name && result.replace(child).is_some() {
                return Err(Error::Format);
            }
        }
        result.ok_or(Error::Missing)
    }
}

pub(super) struct Children<'a> {
    cursor: Cursor<'a>,
    done: bool,
}

impl<'a> Children<'a> {
    fn next_child(&mut self) -> Result<Option<Node<'a>>> {
        loop {
            match self.cursor.next()? {
                Token::Begin(name) => {
                    let child = Node {
                        tree: self.cursor.tree,
                        name,
                        start: self.cursor.position,
                    };
                    let mut depth = 1;
                    while depth != 0 {
                        match self.cursor.next()? {
                            Token::Begin(_) => depth += 1,
                            Token::End => depth -= 1,
                            Token::Finish => return Err(Error::Format),
                            _ => {}
                        }
                    }
                    return Ok(Some(child));
                }
                Token::End => return Ok(None),
                Token::Finish => return Err(Error::Format),
                _ => {}
            }
        }
    }
}

impl<'a> Iterator for Children<'a> {
    type Item = Result<Node<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.next_child() {
            Ok(Some(node)) => Some(Ok(node)),
            result => {
                self.done = true;
                result.err().map(Err)
            }
        }
    }
}
