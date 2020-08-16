require 'test_helper'

class RbfbclientStartTest < Minitest::Test
  def test_has_version
    assert Rbfbclient::VERSION != nil
  end

  def test_start
    assert_equal Rbfbclient.teste(""), "???"
  end
end
