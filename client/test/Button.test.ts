import { afterEach, describe, it, expect, vi, beforeEach } from 'vitest';
import { cleanup, fireEvent, render } from '@testing-library/svelte';

import Button from '../src/lib/components/Button.svelte';

const BUTTON_DEFAULT_CLASSES =
  'bg-black px-5 py-3 text-base font-medium text-center text-white bg-primary-700 rounded-lg hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 sm:w-auto dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800';

describe('$lib/components/Button.svelte', () => {
  beforeEach(() => cleanup());

  it('creates a button with default props', () => {
    const { container } = render(Button);
    const buttonElement = container
      .getElementsByTagName('button')
      .item(0) as HTMLButtonElement;

    expect(buttonElement).toBeTruthy();
    expect(buttonElement.getAttribute('type')).toStrictEqual('button');
    expect(buttonElement.getAttribute('disabled')).toBeNull();
  });

  it('creates a disabled button', () => {
    const { container } = render(Button, { type: 'button', disabled: true });
    const buttonElement = container
      .getElementsByTagName('button')
      .item(0) as HTMLButtonElement;

    expect(buttonElement).toBeTruthy();
    expect(buttonElement.getAttribute('type')).toStrictEqual('button');
    expect(buttonElement.getAttribute('disabled')).toBeDefined();
  });

  it('creates a submit type button', () => {
    const { container } = render(Button, { type: 'submit' });
    const buttonElement = container
      .getElementsByTagName('button')
      .item(0) as HTMLButtonElement;

    expect(buttonElement).toBeTruthy();
    expect(buttonElement.getAttribute('type')).toStrictEqual('submit');
  });

  it('holds classes by default', () => {
    const { container } = render(Button);
    const buttonElement = container
      .getElementsByTagName('button')
      .item(0) as HTMLButtonElement;

    expect(buttonElement).toBeTruthy();
    expect(buttonElement.getAttribute('class')).toStrictEqual(
      [BUTTON_DEFAULT_CLASSES].join(' ')
    );
  });

  it('extends default classes when disabled', () => {
    const { container } = render(Button, { disabled: true });
    const buttonElement = container
      .getElementsByTagName('button')
      .item(0) as HTMLButtonElement;

    expect(buttonElement).toBeTruthy();
    expect(buttonElement.getAttribute('class')).toStrictEqual(
      [BUTTON_DEFAULT_CLASSES].join(' ')
    );
  });

  it('extends default classes to have full width', () => {
    const { container } = render(Button, { fullWidth: true });
    const buttonElement = container
      .getElementsByTagName('button')
      .item(0) as HTMLButtonElement;

    expect(buttonElement).toBeTruthy();
    expect(buttonElement.getAttribute('class')).toStrictEqual(
      [BUTTON_DEFAULT_CLASSES].join(' ')
    );
  });

  it('handles click events', async () => {
    const { container, component } = render(Button);
    const buttonElement = container
      .getElementsByTagName('button')
      .item(0) as HTMLButtonElement;
    const mockFn = vi.fn();

    component.$on('click', mockFn);

    expect(buttonElement).toBeTruthy();
    await fireEvent.click(buttonElement);

    expect(mockFn.mock.calls.length).toEqual(1);
  });
});
