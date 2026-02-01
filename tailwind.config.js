/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'media',
  content: [
    "./src/**/*.{rs,html}",
    "./public/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        // Sakura color palette (pink/cherry blossom)
        sakura: {
          50: 'var(--sakura-50)',
          100: 'var(--sakura-100)',
          200: 'var(--sakura-200)',
          300: 'var(--sakura-300)',
          400: 'var(--sakura-400)',
          500: 'var(--sakura-500)',
          600: 'var(--sakura-600)',
          700: 'var(--sakura-700)',
          800: 'var(--sakura-800)',
          900: 'var(--sakura-900)',
        },
        // 90s web neon colors
        webring: {
          cyan: '#00FFFF',
          magenta: '#FF00FF',
          lime: '#00FF00',
          yellow: '#FFFF00',
        }
      },
      fontFamily: {
        'comic': ['Comic Sans MS', 'cursive'],
        'system': ['Arial', 'Helvetica', 'sans-serif'],
        'mono': ['Courier New', 'monospace'],
      },
      animation: {
        'blink': 'blink 1s step-end infinite',
        'marquee': 'marquee 20s linear infinite',
      },
      keyframes: {
        blink: {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0' },
        },
        marquee: {
          '0%': { transform: 'translateX(100%)' },
          '100%': { transform: 'translateX(-100%)' },
        },
      },
      boxShadow: {
        'inset-90s': 'inset 2px 2px 0 rgba(255,255,255,.5), inset -2px -2px 0 rgba(0,0,0,.5)',
        'raised-90s': '2px 2px 0 rgba(0,0,0,.5), 4px 4px 0 rgba(0,0,0,.25)',
      }
    },
  },
  plugins: [],
}
