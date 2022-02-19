"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault");

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports["default"] = DirectionClues;

var _react = _interopRequireDefault(require("react"));

var _propTypes = _interopRequireDefault(require("prop-types"));

var _Clue = _interopRequireDefault(require("./Clue"));

// import styled from 'styled-components';
function DirectionClues(_ref) {
  var direction = _ref.direction,
      clues = _ref.clues;
  return /*#__PURE__*/_react["default"].createElement("div", {
    className: "direction"
  }, /*#__PURE__*/_react["default"].createElement("h3", {
    className: "header"
  }, direction.toUpperCase()), clues.map(function (_ref2) {
    var number = _ref2.number,
        clue = _ref2.clue,
        correct = _ref2.correct;
    return /*#__PURE__*/_react["default"].createElement(_Clue["default"], {
      key: number,
      direction: direction,
      number: number,
      correct: correct
    }, clue);
  }));
}

process.env.NODE_ENV !== "production" ? DirectionClues.propTypes = {
  /** direction of this list of clues ("across" or "down") */
  direction: _propTypes["default"].string.isRequired,

  /** clues for this List's direction */
  clues: _propTypes["default"].arrayOf(_propTypes["default"].shape({
    /** number of the clue (the label shown) */
    number: _propTypes["default"].string.isRequired,

    /** clue text */
    clue: _propTypes["default"].node.isRequired,

    /** whether the answer/guess is correct */
    correct: _propTypes["default"].bool,
    isFilled: _propTypes["default"].bool,
    filledCounter: _propTypes["default"].number
  })).isRequired
} : void 0;
DirectionClues.defaultProps = {};