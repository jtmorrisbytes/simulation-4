const parser = /^postgres:\/\/(?:([A-Za-z0-9]+)\:)?(?:([0-9A-Za-z]+)@)?(?:([A-Za-z0-9\.\-]+))?(?:\:(\d+))?(?:\/([a-zA-Z0-9]+))?/;
const PROTOCOL = "postgres://";

const MATCH = 0;
const USERNAME = 1;
const PASSWORD = 2;
const HOST = 3;
const PORT = 4;
const DATABASE = 5;

module.exports = function parsePostgresUrl(url = "") {
  if (typeof url !== "string") {
    throw new TypeError("parsePostgresUrl expects a string");
  }
  if (!url.startsWith(PROTOCOL)) {
    throw new SyntaxError("Postgres url must start with '" + PROTOCOL + "'");
  }
  let parsed = parser.exec(url);
  return {
    match: parsed[MATCH],
    user: parsed[USERNAME] || "",
    password: parsed[PASSWORD] || "",
    host: parsed[HOST] || "",
    port: parsed[PORT] || "",
    database: parsed[DATABASE] || "",
  };
};
