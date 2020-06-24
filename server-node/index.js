require("dotenv").config();
const express = require("express");
const massive = require("massive");
const parsePostgresUrl = require("./parsePostgresUrl");

let { DATABASE_URL } = process.env;

const DATABASE = parsePostgresUrl(DATABASE_URL);

console.log(DATABASE);

massive({
  ...DATABASE,
  ssl: { mode: "require", rejectUnauthorized: false },
}).then((db) => {
  console.log(db);
});
