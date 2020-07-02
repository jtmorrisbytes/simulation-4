const bcrypt = require("bcrypt");
const massive = require("massive");
function register(req, res, next) {
  console.log("register called");
  try {
    bcrypt.genSalt(12, (err, salt) => {
      if (err) throw err;
      bcrypt.hash(req.body.password, salt, (err, hash) => {
        if (err) throw err;
        console.log("password hash", hash);
        req.app
          .get("db")
          .user.register(req.body.username, hash, req.body.profile)
          .then((result) => {
            res.status(201).send("Registration Successful");
          })
          .catch((err) => {
            if (err.code == 23505) {
              console.error("user already exists, responding with 201");
              res.status(201).send("Registration Successful");
            } else {
              console.error(err);
              res.status(500).send();
            }
          });
      });
    });
  } catch (err) {
    console.error("an error occured!!!", err);
    next(err);
  }
}
function login(req, res, next) {
  try {
    req.app
      .get("db")
      .user.getByUsername(req.body.username)
      .then((result) => {
        let user = result[0] || {};
        bcrypt.compare(
          req.body.password,
          user.password || "",
          (err, result) => {
            if (err || !result) {
              res.status(401).send("Incorrect username or password");
            } else {
              res.json({
                id: user.id,
                username: user.username,
                profile: user.profile || "null",
              });
            }
          }
        );
      });
    // res.status(501).json(501);
  } catch (err) {
    next(err);
  }
}

module.exports = { login, register };
