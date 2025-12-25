/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.rs", "./index.html"],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                background: '#f8f9fa',
                surface: '#ffffff',
                primary: '#3b82f6',
                secondary: '#64748b',
            }
        },
    },
    plugins: [
        require('tailwindcss-animate'),
    ],
}
