require("dotenv").config();
const express = require("express");
const massive = require("massive");

const parsePostgresUrl = require("./parsePostgresUrl");

const api = require("./routes");

let { DATABASE_URL } = process.env;

// const DATABASE = parsePostgresUrl(DATABASE_URL);

massive({
  connectionString: DATABASE_URL,
  ssl: { mode: "require", rejectUnauthorized: false },
}).then((db) => {
  api.set("db", db);
  api.listen(8000, "0.0.0.0", () => {
    console.log(`listening on http://0.0.0.0:8000`);
  });
});
