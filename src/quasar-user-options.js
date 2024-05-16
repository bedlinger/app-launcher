import { Dark } from "quasar";
import "quasar/dist/quasar.css";
import "@quasar/extras/material-icons/material-icons.css";

// To be used on app.use(Quasar, { ... })
export default {
  config: {
    brand: {
      primary: "#3758ef",
      secondary: "#26A69A",
      accent: "#9C27B0",

      dark: "#1d1d1d",
      "dark-page": "#121212",

      positive: "#21BA45",
      negative: "#C10015",
      info: "#31CCEC",
      warning: "#F2C037",
    },
  },
  plugins: {
    Dark,
  },
};