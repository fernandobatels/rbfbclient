require "rbfbclient/version"
require 'rutie'

module Rbfbclient
  Rutie.new(:rbfbclient).init("Init_rbfbclient", __dir__)

  Connection = ::Connection

  Connection.class_eval do
    def execute(query, *params)
      # In rust layer we can't use variable arguments,
      # so in ruby layer we always cast to a single array

      if params.size == 1 and params[0].kind_of?(Array)
        params = params[0]
      end

      self._execute(query, params)
    end
  end
end
