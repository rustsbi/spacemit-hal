"""Wraps a raw SRAM image using SpacemiT's unmodified development-image tool."""

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


def prepare(node, source, destination):
    if isinstance(node, list):
        for child in node:
            prepare(child, source, destination)
    elif isinstance(node, dict):
        if "pubkey" in node:
            key = node["pubkey"]
            original = source / key["source"]
            if not original.is_file():
                raise ValueError(f"Missing vendor development key: {original}")
            target = destination / key["source"]
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(original, target)
        if "file" in node and node["file"].get("name") == "fsbl":
            node["file"]["source"] = "rot-bootloader.bin"
        for child in node.values():
            prepare(child, source, destination)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--vendor-uboot", required=True, type=Path)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--openssl", type=Path)
    args = parser.parse_args()
    vendor = args.vendor_uboot.resolve()
    output = args.output.resolve()
    raw = args.binary.read_bytes()
    if not 0 < len(raw) <= 0x20000:
        raise ValueError("Raw image must fit the example's 128 KiB SRAM image region")
    if output.exists():
        raise ValueError("Output already exists")
    if args.openssl:
        if not args.openssl.is_file():
            raise ValueError("OpenSSL executable does not exist")
        os.environ["PATH"] = str(args.openssl.resolve().parent) + os.pathsep + os.environ["PATH"]
    if not args.openssl and shutil.which("openssl") is None:
        raise ValueError("The vendor tool requires openssl on PATH")
    config_dir = vendor / "board/spacemit/k1-x/configs"
    config = json.loads((config_dir / "fsbl.json").read_text(encoding="utf-8"))
    with tempfile.TemporaryDirectory(prefix="rot-fsbl-", dir=output.parent) as temp:
        directory = Path(temp)
        # The vendor tool splits OpenSSL arguments on whitespace.
        if any(c.isspace() for c in str(directory)):
            raise ValueError("Use an output directory without whitespace")
        prepare(config, config_dir, directory)
        (directory / "rot-bootloader.bin").write_bytes(raw)
        (directory / "fsbl.json").write_text(json.dumps(config), encoding="utf-8")
        subprocess.run([sys.executable, "-S", str(vendor / "tools/build_binary_file.py"),
                        "-c", str(directory / "fsbl.json"),
                        "-o", str(directory / "FSBL.bin")], cwd=directory, check=True)
        image = (directory / "FSBL.bin").read_bytes()
        padded = (len(raw) + 31) & ~31
        if (len(image) != 0x1000 + padded + 256
                or image[0x100:0x108] != b"AIHD\x01\0\0\0"
                or image[0xFE0:0xFE8] != b"AIHD\x01\0\0\0"
                or int.from_bytes(image[0x108:0x110], "little") != 0x1000
                or int.from_bytes(image[0xFE8:0xFF0], "little") != padded
                or image[0x1000:0x1000 + len(raw)] != raw):
            raise ValueError("Vendor output failed K1 development-container checks")
        # Verify both certificates using the vendor development public keys.
        for key, data, signature in [
            ("rsakeypair0_prv.key", image[0x100:0xB00], image[0xB00:0xC00]),
            ("spl_pubkey_prv.key", image[0xFE0:-256], image[-256:]),
        ]:
            public = directory / "verify-public.pem"
            message = directory / "verify-message.bin"
            signed = directory / "verify-signature.bin"
            message.write_bytes(data)
            signed.write_bytes(signature)
            subprocess.run(["openssl", "rsa", "-in", str(directory / "key" / key),
                            "-pubout", "-out", str(public)], check=True)
            subprocess.run(["openssl", "dgst", "-sha256", "-verify", str(public),
                            "-signature", str(signed), str(message)], check=True)
        with output.open("xb") as file:
            file.write(image)
    print(f"{output}: {len(image)} bytes; development keys, not secure boot")


if __name__ == "__main__":
    main()
