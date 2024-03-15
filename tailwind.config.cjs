/** @type {import('tailwindcss').Config}*/
const config = {
	content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  
	plugins: [require('flowbite/plugin')],
  
	darkMode: 'class',
  
	theme: {
	  extend: {
		colors: {
		  // flowbite-svelte
		  primary: {
			50: '#FFF5F2',
			100: '#FFF1EE',
			200: '#FFE4DE',
			300: '#FFD5CC',
			400: '#FFBCAD',
			500: '#FE795D',
			600: '#EF562F',
			700: '#EB4F27',
			800: '#CC4522',
			900: '#A5371B'
		  }
		},
		animation: {
			'err-bounce': 'err 200ms ease 1'
		},
		keyframes: {
			err: {
				'0%, 100%': { transform: 'translateX(0)' },
				'25%': { transform: 'translateX(5%)' },
				'75%': { transform: 'translateX(-5%)' },
			}
		}
	  }
	}
  };
  

module.exports = config;
