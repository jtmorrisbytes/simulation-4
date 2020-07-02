const { response } = require("../routes");
// const express = require("express");
function create(req, res) {
  console.log("create post", req.body, req.params.userId);
  res.status(422);
  if (!req.is("application/json")) {
    res.send("Invalid content-type, expecting application/json");
  } else if (typeof req.body !== "object") {
    res.send("Invalid or empty request body, expecting text");
  } else {
    try {
      req.app
        .get("db")
        .post.create(
          req.body.title,
          req.body.image,
          req.body.content,
          req.params.userId
        )
        .then(
          (r) => res.status(201).json(r[0] || r),
          (e) => {
            console.error(e);
            res.status(500).json({ error: e.message || e });
          }
        );
      // res.status(201).send("hello from create posts");
    } catch (e) {
      console.log(e);
      res.status(501).send(e.message);
    }
  }
}

function search(req, res, next) {
  let search = String(req.body.search || "");
  let result = [];
  console.log(req.body.search, req.params.userId);
  if (req.body.userposts === true) {
    req.app
      .get("db")
      .post.searchUser(req.params.userId, `%${req.body.search}%`)
      .then(
        (l) => res.status(200).json(l),
        (e) => res.status(500).json(e)
      );
    // res.status(200).send("you searched user posts for " + search);
  } else {
    req.app
      .get("db")
      .post.search(req.body.search)
      .then(
        (l) => res.status(200).json(l),
        (e) => {
          res.status(500).json(e);
        }
      );
    // res.status(200).send("you searched for " + search);
  }
}

function get(req, res) {
  req.app
    .get("db")
    .post.getById(req.params.postId)
    .then(
      (r) => res.status(200).json(r[0] || null),
      (e) => {
        console.error("something went wrong", e);
        res.status(500).json(e);
      }
    );
}

module.exports = { create, search, get };
