$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)
require "rbfbclient"

require "minitest/autorun"
require 'color_pound_spec_reporter'
Minitest::Reporters.use! [ColorPoundSpecReporter.new]   
