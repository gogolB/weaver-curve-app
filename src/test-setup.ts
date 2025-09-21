import '@testing-library/jest-dom/vitest';
import { vi } from 'vitest';

// Mock Tauri API since tests run in Node.js environment
(globalThis as any).window = Object.assign((globalThis as any).window || {}, {
  __TAURI_INTERNALS__: {},
});

// Mock Tauri functions
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/app', () => ({
  getVersion: vi.fn().mockResolvedValue('1.4.0'),
}));

vi.mock('@tauri-apps/plugin-dialog', () => ({
  save: vi.fn(),
}));