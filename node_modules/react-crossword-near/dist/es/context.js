"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault");

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.CrosswordSizeContext = exports.CrosswordContext = void 0;

var _react = _interopRequireDefault(require("react"));

// To pass focus/highlight/etc., it's cleaner to use a context.
var CrosswordContext = /*#__PURE__*/_react["default"].createContext({
  focused: false,
  selectedDirection: null,
  selectedNumber: null // correct answers?

});

exports.CrosswordContext = CrosswordContext;

var CrosswordSizeContext = /*#__PURE__*/_react["default"].createContext({
  cellSize: 0,
  cellPadding: 0,
  cellInner: 0,
  cellHalf: 0,
  fontSize: 0
});

exports.CrosswordSizeContext = CrosswordSizeContext;