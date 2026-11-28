export interface TargetSpec {
  triple: string;
  archive: string;
  format: "tar.gz" | "zip";
  sha256: string;
  bins: Record<string, string>;
}

export interface Manifest {
  version: string;
  tag: string;
  releaseBaseUrl: string;
  binaries: string[];
  targets: Record<string, TargetSpec>;
}

export interface EnsureInstalledOptions {
  manifestPath?: string;
  manifest?: Manifest;
  platformKey?: string;
  baseUrl?: string;
  env?: Record<string, string | undefined>;
  force?: boolean;
}

export interface InstalledInfo {
  cached: boolean;
  platformKey: string;
  triple: string;
  version: string;
  installDir: string;
}

export function loadManifest(customPath?: string): Manifest;
export function currentPlatformKey(overrides?: { platform?: string; arch?: string }): string;
export function resolveTarget(
  manifest: Manifest,
  platformKey?: string,
): { key: string; target: TargetSpec } | null;
export function cacheDir(envObj?: Record<string, string | undefined>): string;
export function ensureInstalled(options?: EnsureInstalledOptions): Promise<InstalledInfo>;
export function binaryPath(binaryName: string, options?: EnsureInstalledOptions): Promise<string>;
export function runBinary(
  binaryName: string,
  argv: string[],
  options?: EnsureInstalledOptions,
): Promise<number>;
