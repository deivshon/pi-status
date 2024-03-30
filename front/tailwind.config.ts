/** @type {import('tailwindcss').Config} */
export default {
    content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
    theme: {
        extend: {
            colors: {
                "ayu-purple": "#DFBFFF",
                "ayu-green": "#95E6CB",
                "ayu-red": "#F28779",
                "ayu-cyan": "#6CCDFF",
                "ayu-yellow": "#EEBA45",
                "proc-search-bar": "#0B0E14",
                lightgrey: "#D3D3D3",
                progress: "#565B66",
            },
        },
    },
    plugins: [],
};
