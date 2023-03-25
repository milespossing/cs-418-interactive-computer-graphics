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
        },
        "lightPosX": {
          "type": "number",
          "label": "Light Position X",
          "default": 1
        },
        "lightPosY": {
          "type": "number",
          "label": "Light Position Y",
          "default": 1
        },
        "lightPosZ": {
          "type": "number",
          "label": "Light Position Z",
          "default": 1
        },
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
