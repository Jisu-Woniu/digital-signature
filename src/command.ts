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
  unavailable = 3,
  other = 2,
}

export const detectFileType = (path: string) =>
  invoke<FileType>("detect_file_type", { path });

export const signFiles = (
  filePaths: string[],
  privateKeyPath: string,
  passwd: string,
) =>
  invoke<string[]>("sign_files", {
    filePaths,
    privateKeyPath,
    passwd,
  });

export const getFileNames = (files: string[]) =>
  invoke<string[]>("get_file_names", { files });

export type VerificationResult = Record<string, boolean>;

export const verifySignatures = (
  signaturePaths: string[],
  publicKeyPath: string,
) =>
  invoke<VerificationResult>("verify_signatures", {
    signaturePaths,
    publicKeyPath,
  });
