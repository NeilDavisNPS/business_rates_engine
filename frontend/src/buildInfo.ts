declare const __APP_VERSION__: string;
declare const __BUILD_NUMBER__: string;
declare const __COMMIT_HASH__: string;
declare const __BUILD_TIMESTAMP__: string;

export interface BuildInfo {
  version: string;
  buildNumber: string;
  commitHash: string;
  timestamp: string;
  formattedTimestamp: string;
}

export function formatUtcTimestamp(isoStr: string): string {
  try {
    const d = new Date(isoStr);
    if (isNaN(d.getTime())) return isoStr;
    const year = d.getUTCFullYear();
    const month = String(d.getUTCMonth() + 1).padStart(2, '0');
    const day = String(d.getUTCDate()).padStart(2, '0');
    const hours = String(d.getUTCHours()).padStart(2, '0');
    const minutes = String(d.getUTCMinutes()).padStart(2, '0');
    const seconds = String(d.getUTCSeconds()).padStart(2, '0');
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds} UTC`;
  } catch {
    return isoStr;
  }
}

export const buildInfo: BuildInfo = {
  version: typeof __APP_VERSION__ !== 'undefined' ? __APP_VERSION__ : '0.0.1',
  buildNumber: typeof __BUILD_NUMBER__ !== 'undefined' ? __BUILD_NUMBER__ : '0',
  commitHash: typeof __COMMIT_HASH__ !== 'undefined' ? __COMMIT_HASH__ : 'dev',
  timestamp: typeof __BUILD_TIMESTAMP__ !== 'undefined' ? __BUILD_TIMESTAMP__ : new Date().toISOString(),
  get formattedTimestamp(): string {
    return formatUtcTimestamp(this.timestamp);
  },
};

