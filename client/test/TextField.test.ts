import { describe, expect, it } from 'vitest';
import { render } from '@testing-library/svelte';

import TextField from '../src/lib/components/TextField.svelte';

const INPUT_DEFAULT_CLASSES =
  'bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-500 focus:border-primary-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500';

describe('Test for TextField component', () => {
  it('Create input with default props', () => {
    const { container } = render(TextField);
    const inputElement = container
      .getElementsByTagName('input')
      .item(0) as HTMLElement;
    const paragraphErrorElement = container
      .getElementsByTagName('p')
      .item(0) as HTMLParagraphElement;
    const labelElement = container
      .getElementsByTagName('label')
      .item(0) as HTMLLabelElement;

    expect(inputElement).toBeTruthy();
    expect(inputElement.getAttribute('type')).toBe('text');
    expect(inputElement.getAttribute('value')).toBe(null);
    expect(inputElement.getAttribute('placeholder')).toBe(null);
    expect(inputElement.getAttribute('autocomplete')).toBe(null);
    expect(paragraphErrorElement).toBeTruthy();
    expect(labelElement).toBeFalsy();
  });

  it('should render a label when the `label` prop is passed', () => {
    const { container } = render(TextField, { props: { label: 'Name' } });

    const labelElement = container.querySelector('label');
    expect(labelElement).toBeTruthy();

    expect(labelElement?.textContent).toBe('Name');
  });

  it('applies custom classes to the input', () => {
    const { container } = render(TextField, {
      props: {
        class: 'custom-class-1'
      }
    });

    const inputElement = container
      .getElementsByTagName('input')
      .item(0) as HTMLInputElement;

    expect(inputElement.classList.toString()).toBe(
      'custom-class-1 ' + INPUT_DEFAULT_CLASSES
    );
  });

  it('shows error message when error prop is passed', async () => {
    const errorMessage = 'This field is required';
    const { container } = render(TextField, {
      error: errorMessage
    });
    const textField = container
      .getElementsByTagName('p')
      .item(0) as HTMLParagraphElement;

    expect(textField.textContent).toBe(errorMessage);
  });
});
