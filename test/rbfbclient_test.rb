require 'test_helper'

class RbfbclientTest < Minitest::Test
  def test_has_version
    assert !Rbfbclient::VERSION.nil?
  end

  def test_simple_conn
    conn = Rbfbclient::Connection.new({
                                        host: 'localhost',
                                        port: 3050,
                                        db_name: 'test.fdb',
                                        user: 'SYSDBA',
                                        pass: 'masterkey'
                                      })
    assert !conn.nil?
    conn.close
  end
end
