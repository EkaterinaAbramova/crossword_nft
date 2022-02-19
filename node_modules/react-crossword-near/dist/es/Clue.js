"use strict";

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard");

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault");

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports["default"] = Clue;

var _extends2 = _interopRequireDefault(require("@babel/runtime/helpers/extends"));

var _objectWithoutPropertiesLoose2 = _interopRequireDefault(require("@babel/runtime/helpers/objectWithoutPropertiesLoose"));

var _react = _interopRequireWildcard(require("react"));

var _propTypes = _interopRequireDefault(require("prop-types"));

var _styledComponents = _interopRequireWildcard(require("styled-components"));

var _context = require("./context");

var ClueWrapper = _styledComponents["default"].div.attrs(function (props) {
  return {
    className: "clue" + (props.correct ? ' correct' : '')
  };
}).withConfig({
  displayName: "Clue__ClueWrapper",
  componentId: "sc-6vr1y0-0"
})(["cursor:default;background-color:", ";"], function (props) {
  return props.highlight ? props.highlightBackground : 'transparent';
});

function Clue(_ref) {
  var direction = _ref.direction,
      number = _ref.number,
      children = _ref.children,
      correct = _ref.correct,
      props = (0, _objectWithoutPropertiesLoose2["default"])(_ref, ["direction", "number", "children", "correct"]);

  var _useContext = (0, _react.useContext)(_styledComponents.ThemeContext),
      highlightBackground = _useContext.highlightBackground;

  var _useContext2 = (0, _react.useContext)(_context.CrosswordContext),
      focused = _useContext2.focused,
      selectedDirection = _useContext2.selectedDirection,
      selectedNumber = _useContext2.selectedNumber,
      onClueSelected = _useContext2.onClueSelected;

  var handleClick = (0, _react.useCallback)(function (event) {
    event.preventDefault();

    if (onClueSelected) {
      onClueSelected(direction, number);
    }
  }, [direction, number, onClueSelected]);
  return /*#__PURE__*/_react["default"].createElement(ClueWrapper, (0, _extends2["default"])({
    highlightBackground: highlightBackground,
    highlight: focused && direction === selectedDirection && number === selectedNumber,
    correct: correct
  }, props, {
    onClick: handleClick,
    "aria-label": "clue-" + number + "-" + direction
  }), number, ": ", children);
}

process.env.NODE_ENV !== "production" ? Clue.propTypes = {
  /** direction of the clue: "across" or "down"; passed back in onClick */
  direction: _propTypes["default"].string.isRequired,

  /** number of the clue (the label shown); passed back in onClick */
  number: _propTypes["default"].string.isRequired,

  /** clue text */
  children: _propTypes["default"].node,

  /** whether the answer/guess is correct */
  correct: _propTypes["default"].bool,
  isFilled: _propTypes["default"].bool
} : void 0;
Clue.defaultProps = {
  children: undefined,
  correct: undefined,
  isFilled: undefined
};