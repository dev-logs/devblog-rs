const plugin = require('tailwindcss/plugin');

module.exports = plugin(function ({ addUtilities, theme }) {
    const glowEffect = (color) => ({
        backgroundImage: `radial-gradient(circle at center, ${theme(`colors.${color}.500`)} 1px, transparent 0), radial-gradient(circle at center, ${theme(`colors.${color}.500`)} 1px, transparent 0)`,
        backgroundSize: '10px 10px',
        backgroundPosition: '0 0, 5px 5px',
    });

    const utilities = Object.fromEntries(
        Object.keys(theme('colors')).map((color) => [
            `.glow-effect-${color}`,
            glowEffect(color),
        ])
    );

    addUtilities(utilities);
});