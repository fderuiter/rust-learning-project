import { expect, test, vi } from 'vitest';
import { loadWasm } from '../wasm-loader.js';
import init from '../../rust_learning_project.js';

vi.mock('../../rust_learning_project.js', () => {
    return {
      default: vi.fn(),
    }
});

test('loadWasm should call init', async () => {
    await loadWasm();
    expect(init).toHaveBeenCalled();
});
