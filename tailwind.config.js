/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "index.html",
    "./src/**/*.{js,jsx}",
    "./src/App.jsx"], 
  theme: {
    extend: {},
  },
  plugins: [],
  corePlugins: {
    preflight: false // <== disable this!
  },
}

