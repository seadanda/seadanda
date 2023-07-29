/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{rs,html,css}"],
    theme: {
        extend: {
            colors: {
                corcra: {
                    50: "#efeaf6",
                    100: "#ded5ee",
                    200: "#bdabdd",
                    300: "#9c81cb",
                    400: "#7b57ba",
                    500: "#5a2da9",
                    600: "#482487",
                    700: "#361b65",
                    800: "#241244",
                    900: "#120922",
                },
                gorm: {
                    50: "#eaf2fb",
                    100: "#d6e4f6",
                    200: "#accaee",
                    300: "#83afe5",
                    400: "#5995dd",
                    500: "#307ad4",
                    600: "#2662aa",
                    700: "#1d497f",
                    800: "#133155",
                    900: "#0a182a",
                },
            },
            fontFamily: ["Poppins", "sans-serif"],
        },
    },
    plugins: [],
};
