/** @type {import('tailwindcss').Config} */
export default {
    content: [
            './index.html',
            './src/**/*.{html,js,svelte,ts}',
            './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
    ],
    theme: {
    extend: {},
    },
    plugins: [require('flowbite/plugin')]
}

