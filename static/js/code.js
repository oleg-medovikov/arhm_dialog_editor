   import { idFetcher } from "/idFetcher.js";

   const $ = go.GraphObject.make;

   const UI = {
     graph: document.getElementById("graph"),
     save: document.getElementById("save"),
     load: document.getElementById("load"),
     json: document.getElementById("json"),
   };

   const graph = new go.Diagram(UI.graph, {
     "toolManager.mouseWheelBehavior": go.WheelMode.Zoom,
     "clickCreatingTool.archetypeNodeData": { text: "текст" },
  "undoManager.isEnabled": true,
});

// Функция генерации id должна синхронно возвращать строку или число.
graph.model.makeUniqueKeyFunction = () => idFetcher.next();
// Задаём название поля айдишника, по умолчанию это "key"
graph.model.nodeKeyProperty = "id";

graph.nodeTemplate = $(
  go.Node,
  "Auto",
  $(go.Shape, "RoundedRectangle", {
    fill: "white",
    portId: "",
    fromLinkable: true,
    fromLinkableSelfNode: false,
    fromLinkableDuplicates: true,
    toLinkable: true,
    toLinkableSelfNode: false,
    toLinkableDuplicates: true,
    cursor: "pointer",
  }),
  $(go.TextBlock, { margin: 8, editable: true }).bindTwoWay("text")
)
  // Сохранять позиции нод?
  .bindTwoWay("location", "loc", go.Point.parse, go.Point.stringify);

graph.nodeTemplate.selectionAdornmentTemplate = $(
  go.Adornment,
  "Spot",
  $(
    go.Panel,
    "Auto",
    $(go.Shape, "RoundedRectangle", { fill: null, stroke: "skyblue" }),
    $(go.Placeholder)
  ),
  $(
    "Button",
    {
      alignment: go.Spot.TopRight,
      click: onAddClick,
    },
    $(go.Shape, "PlusLine", { width: 8, height: 8 })
  )
);

/**
 * @param {go.InputEvent} e
 * @param {{part: go.Adornment}} obj
 */
function onAddClick(e, obj) {
  const adornment = obj.part;
  graph.startTransaction("Add State");
  const fromNode = adornment.adornedPart;
  const fromData = fromNode.data;
  const point = fromNode.location.copy();
  point.x += 200;
  const toData = {
    text: "текст",
    loc: go.Point.stringify(point),
  };
  graph.model.addNodeData(toData);
  const linkData = {
    from: graph.model.getKeyForNodeData(fromData),
    to: graph.model.getKeyForNodeData(toData),
    text: "привет",
  };
  graph.model.addLinkData(linkData);
  graph.commitTransaction("Add State");
  const newNode = graph.findNodeForData(toData);
  graph.select(newNode);
  graph.scrollToRect(newNode.actualBounds);
}

// шаблон стрелки
graph.linkTemplate = $(go.Link, {
  curve: go.Curve.Bezier,
  curviness: 40,
  adjusting: go.LinkAdjusting.Stretch,
  reshapable: true,
  relinkableFrom: true,
  relinkableTo: true,
})
  .bindTwoWay("points")
  .bind("curviness")
  .add(
    // линия стрелки
    $(go.Shape, {
      strokeWidth: 2,
      stroke: "#888",
    }),
    // начало стрелки
    $(go.Shape, {
      fromArrow: "circle",
      fill: "#fff",
    }),
    // конец стрелки
    $(go.Shape, {
      toArrow: "standard",
      stroke: null,
      fill: "#888",
    }),
    // текст стрелки
    $(
      go.Panel,
      "Auto",
      $(go.Shape, "RoundedRectangle", {
        fill: "#ffa",
      }),
      $(go.TextBlock, {
        margin: 4,
        editable: true,
        text: 'привет',
      }))
       .set({ segmentOffset: new go.Point(0, -15) })
      .bindTwoWay("text")
  );

function save() {
  const jsonString = graph.model.toJson();
  const decodedString = jsonString.replace(/\\u([\dA-Fa-f]{4})/g, (_, hex) =>
    String.fromCharCode(parseInt(hex, 16))
  );
  UI.json.value = decodedString;
  graph.isModified = false;
}
UI.save.addEventListener("click", save);

function load() {
  const encodedJsonString = UI.json.value.replace(
    /[\u0400-\u04FF]/g,
    (char) => `\\u${char.charCodeAt(0).toString(16).padStart(4, "0")}`
  );
  graph.model = go.Model.fromJson(encodedJsonString);
}
UI.load.addEventListener("click", load);
