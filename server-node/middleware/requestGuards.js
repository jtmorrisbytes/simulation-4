function expectUserIdInParams(req, res, next) {
  if (req.params.userId === undefined) {
    res.status(400).send("missing userId in request");
  } else if (String(parseInt(req.params.userId)) === "NaN") {
    res
      .status(422)
      .send(
        "expecting userId to be an integer, recieved " +
          typeof req.params.userId
      );
  } else if (req.params.userId < 0) {
    res
      .status(400)
      .send(
        "expecting userId to be greater than zero, recieved " +
          req.params.userId
      );
  } else {
    next();
  }
}

function expectJsonBody(req, res, next) {
  if (!req.is("application/json")) {
    res.status(400).json("expecting content type application/json");
  } else {
    try {
      JSON.parse(req.body);
    } catch (parseError) {
      res
        .status(422)
        .json(`JSON parse error:  ${parseError.message || parseError}`);
      return;
    }
    next();
  }
}
module.exports = { expectUserIdInParams, expectJsonBody };
