/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {
            fontFamily: {
                main: ['Montserrat', 'sans-serif'],
                code: ['Source Code Pro', 'sans-serif']
            },
            colors: {
                primaryC: '#000000CC',          // primaryContainerColor
                onPrimaryC: '#F5F7F8',          // onPrimaryContainerColor
                backgroundC: '#000',            // backgroundColor
                onBackgroundC: '#F5F7F8',       // onBackgroundColor
                secondaryC: '#191919',          // secondaryContainerColor
                onSecondaryC: '#F4CE14',        // onSecondaryContainerColor
            },
            spacing: {
                '1/2': '50%',
                '1/3': '33.333333%',
                '2/3': '66.666667%',
                '1/4': '25%',
                '2/4': '50%',
                '3/4': '75%',
                '1/5': '20%',
                '2/5': '40%',
                '3/5': '60%',
                '4/5': '80%',
                '1/6': '16.666667%',
                '2/6': '33.333333%',
                '3/6': '50%',
                '4/6': '66.666667%',
                '5/6': '83.333333%',
                '1/12': '8.333333%',
                '2/12': '16.666667%',
                '3/12': '25%',
                '4/12': '33.333333%',
                '5/12': '41.666667%',
                '6/12': '50%',
                '7/12': '58.333333%',
                '8/12': '66.666667%',
                '9/12': '75%',
                '10/12': '83.333333%',
                '11/12': '91.666667%',
                '200': '200px',
                '200px': '200px',
                '220px': '220px',
                '50px': '50px'
            }
        },
    },
    plugins: [
        require('@tailwindcss/typography')
    ],
}