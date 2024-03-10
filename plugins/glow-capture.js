/** @type {Plugin} */

const plugin = require("tailwindcss/plugin")

module.exports = plugin(
    function ({ addVariant }) {
        addVariant("glow", ".glow-capture .glow-overlay &")
    },
    {
        theme: {
            extend: {
                colors: {
                    glow: "color-mix(in srgb, var(--glow-color) calc(<alpha-value> * 100%), transparent)",
                },
            },
        },
    }
)