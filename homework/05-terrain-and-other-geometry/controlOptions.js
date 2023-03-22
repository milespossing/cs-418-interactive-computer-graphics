/**
 * Modify this object to chose what options you want in the control pane.
 * Top-level entries become top-section radio buttons.
 * Nested entries become lower-section inputs of various types.
 */
var controlOptions =
  {
    "model": {
      "label": "Model",
      "options": {
        "model": {
          "type": "radio",
          "options": {
            "monkey.json": "Monkey"
          }
        },
        "shiny": {
          "type": "checkbox",
          "label": "Shiny",
          "default": "true"
        }
      }

    },
    "sphere": {
      "label": "Basic: Sphere",
      "options": {
        "numHorizontal": {
          "type": "number",
          "default": 50,
          "label": "Number of Rings"
        }
      }
    },
    "terrain": {
      "label": "Required: Faulting-method terrain",
      "options": {
        "resolution": {
          "type": "number",
          "default":50,
          "label": "Grid size"
        },
        "slices": {
          "type":"number",
          "default":10,
          "label":"Fractures"
        },
        "shiny": {
          "type": "checkbox",
          "label": "Optional: Shiny",
          "default": "true"
        }
      }
    }
  }
