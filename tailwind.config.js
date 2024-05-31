/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      screens: {
        xs: "420px",
      },
    },
  },
  plugins: [],
};
