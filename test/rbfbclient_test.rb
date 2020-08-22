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

  def test_simplec_exec
    conn = Rbfbclient::Connection.new({
                                        db_name: 'test.fdb',
                                      })
    assert !conn.nil?

    begin
      conn.execute('create table fbtest (name varchar(50))')
    rescue
    end

    conn.execute('insert into fbtest (name) values (\'val test\')')

    conn.close
  end
end
