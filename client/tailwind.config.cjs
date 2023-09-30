const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        base: {
          dark: '#0D1117',
          light: '#F5F5F5'
        },
        inner: {
          dark: '#161B22',
          light: '#0F1419'
        }
      },
      fontFamily: {
        sans: ['Inter', ...defaultTheme.fontFamily.sans]
      }
    },
    plugins: [require('@tailwindcss/forms')]
  }
};
