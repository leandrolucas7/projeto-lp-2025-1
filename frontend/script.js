/***** PRINT BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "print_block",
  "message0": "Imprime %1",
  "args0": [
    {
      "type": "field_input",
      "name": "TEXT"
    }
  ],
  "previousStatement": null,
  "nextStatement": null,
  "colour": 160,
  "tooltip": "Prints output to the console",
  "helpUrl": ""
}]);

/***** IF-ELSE BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "if_else_block",
  "message0": "Se %1 Então %2 Senão %3",
  "args0": [
    {
      "type": "input_value",
      "name": "CONDITION",
      "check": "Boolean"
    },
    {
      "type": "input_statement",
      "name": "IF_BODY"
    },
    {
      "type": "input_statement",
      "name": "ELSE_BODY"
    }
  ],
  "previousStatement": null,
  "nextStatement": null,
  "colour": 210,
  "tooltip": "If-Else conditional",
  "helpUrl": ""
}]);

/***** SUM BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "sum_block",
  "message0": "Soma %1 + %2",
  "args0": [
    {
      "type": "input_value",
      "name": "A",
      "check": "Number"
    },
    {
      "type": "input_value",
      "name": "B",
      "check": "Number"
    }
  ],
  "output": "Number",
  "colour": 230,
  "inputsInline": true,
  "tooltip": "Sum of two numbers",
  "helpUrl": ""
}]);


// INITIALIZE BLOCKLY WORKSPACE
const workspace = Blockly.inject('blocklyDiv', {
  renderer: "zelos", // Prettier format
  trashcan: true,
  toolbox: {
    //kind: "flyoutToolbox",
    kind: "categoryToolbox",
    contents: [
      {
        kind: "category",
        name: "Custom Blocks",
        colour: "#5CA699",
        contents: [
          { kind: "block", type: "print_block" },
          { kind: "block", type: "if_else_block" },
          { kind: "block", type: "sum_block" },
        ]
      },
      {
        kind: "category",
        name: "Logic",
        colour: "%{BKY_LOGIC_HUE}",
        contents: [
          { kind: "block", type: "logic_compare" },
          { kind: "block", type: "logic_boolean" }
        ]
      },
      {
        kind: "category",
        name: "Math",
        colour: "%{BKY_MATH_HUE}",
        contents: [
          { kind: "block", type: "math_number" }
        ]
      }
    ]
  }
});


async function execute() {
    const workspaceJson = Blockly.serialization.workspaces.save(workspace);
    console.log(JSON.stringify(workspaceJson));
    try {
        const response = await fetch('/execute', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(workspaceJson)
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        const output = await response.json();
        console.log(output);

        // Update the <pre id="output">
        document.getElementById('output').textContent = `Output:\n${output.join('\n')}`;

    } catch (error) {
        console.error('Error:', error);
        document.getElementById('output').textContent = `Error: ${error}`;
    }
}
