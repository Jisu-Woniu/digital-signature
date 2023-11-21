import { invoke } from "@tauri-apps/api";

export const generateKeyPair = (name: string, email: string, path: string) =>
  invoke<void>("generate_key_pair", {
    email,
    name,
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
