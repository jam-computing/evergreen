/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const collection = new Collection({
    "id": "5bs0qtkmcdb8sod",
    "created": "2024-05-06 21:15:21.067Z",
    "updated": "2024-05-06 21:15:21.067Z",
    "name": "Tree",
    "type": "base",
    "system": false,
    "schema": [
      {
        "system": false,
        "id": "peb0d8xq",
        "name": "Name",
        "type": "text",
        "required": true,
        "presentable": false,
        "unique": false,
        "options": {
          "min": null,
          "max": null,
          "pattern": ""
        }
      },
      {
        "system": false,
        "id": "cuwb0oe2",
        "name": "LedLocations",
        "type": "json",
        "required": false,
        "presentable": false,
        "unique": false,
        "options": {
          "maxSize": 2000000
        }
      }
    ],
    "indexes": [],
    "listRule": "",
    "viewRule": "",
    "createRule": "",
    "updateRule": "",
    "deleteRule": "",
    "options": {}
  });

  return Dao(db).saveCollection(collection);
}, (db) => {
  const dao = new Dao(db);
  const collection = dao.findCollectionByNameOrId("5bs0qtkmcdb8sod");

  return dao.deleteCollection(collection);
})
