import { invoke } from "@tauri-apps/api";

export interface KeyPairPaths {
  secretKeyPath: string;
  publicKeyPath: string;
}

export const generateKeyPair = (
  name: string,
  email: string,
  passwd: string,
  path: string,
) =>
  invoke<KeyPairPaths>("generate_key_pair", {
    email,
    name,
    passwd,
    path,
  });

export const enum FileType {
  dir = 1,
  file = 0,
  inexist = 3,
  other = 2,
}

export const detectFileType = (path: string) =>
  invoke<FileType>("detect_file_type", { path });
