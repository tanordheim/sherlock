testCase = (require 'nodeunit').testCase
fs = require 'fs'
async = require 'async'
mongoose = require 'mongoose'

configModule = require '../lib/config'
configModule.setConfigDirectory './test/config'
 
SnapshotParser = require '../lib/snapshot_parser'
Snapshot = require '../models/snapshot'
Process = require '../models/process'
MetricLabel = require '../models/metric_label'
Metric = require '../models/metric'
 
validJson = fs.readFileSync "#{__dirname}/assets/agent_data.json", 'utf-8'

#
# Build some invalid JSON structures to use in the tests.
#
# Missing "node" element.
jsonWithoutNode = JSON.parse(validJson)
delete jsonWithoutNode.node
jsonWithoutNode = JSON.stringify(jsonWithoutNode)

# Invalid "node" element.
jsonWithInvalidNode = JSON.parse(validJson)
jsonWithInvalidNode.node = 'invalid'
jsonWithInvalidNode = JSON.stringify(jsonWithInvalidNode)

# Missing agent version.
jsonWithoutAgentVersion = JSON.parse(validJson)
delete jsonWithoutAgentVersion.agent_version
jsonWithoutAgentVersion = JSON.stringify(jsonWithoutAgentVersion)

# Invalid agent version.
jsonWithInvalidAgentVersion = JSON.parse(validJson)
jsonWithInvalidAgentVersion.agent_version = 'invalid'
jsonWithInvalidAgentVersion = JSON.stringify(jsonWithInvalidAgentVersion)

# Missing data.
jsonWithoutData = JSON.parse(validJson)
delete jsonWithoutData.data
jsonWithoutData = JSON.stringify(jsonWithoutData)

# Missing processes.
jsonWithoutProcesses = JSON.parse(validJson)
delete jsonWithoutProcesses.processes
jsonWithoutProcesses = JSON.stringify(jsonWithoutProcesses)

# Invalid processes.
jsonWithInvalidProcesses = JSON.parse(validJson)
jsonWithInvalidProcesses.processes = 'invalid'
jsonWithInvalidProcesses = JSON.stringify(jsonWithInvalidProcesses)

# Empty processes.
jsonWithEmptyProcesses = JSON.parse(validJson)
jsonWithEmptyProcesses.processes = []
jsonWithEmptyProcesses = JSON.stringify(jsonWithEmptyProcesses)

module.exports = testCase
  setUp: (callback) ->
    mongoose.connect 'mongodb://localhost/sherlock_test'
    callback()

  tearDown: (callback) ->
    async.parallel [
        (callback) ->
          Snapshot.collection.remove callback
      , (callback) ->
          Process.collection.remove callback
      , (callback) ->
          MetricLabel.collection.remove callback
      , (callback) ->
          Metric.collection.remove callback
    ], (err, results) ->
      mongoose.disconnect (error) ->
        callback()

  'accept valid json data': (test) ->
    new SnapshotParser validJson, (error) ->
      test.ok !error?
      test.done()

  'do not accept missing node': (test) ->
    new SnapshotParser jsonWithoutNode, (error) ->
      test.ok error?
      test.done()

  'do not accept invalid node': (test) ->
    new SnapshotParser jsonWithInvalidNode, (error) ->
      test.ok error?
      test.done()

  'do not accept missing agent version': (test) ->
    new SnapshotParser jsonWithoutAgentVersion, (error) ->
      test.ok error?
      test.done()

  'do not accept invalid agent version': (test) ->
    new SnapshotParser jsonWithInvalidAgentVersion, (error) ->
      test.ok error?
      test.done()

  'do not accept missing data': (test) ->
    new SnapshotParser jsonWithoutData, (error) ->
      test.ok error?
      test.done()

  'do not accept missing processes': (test) ->
    new SnapshotParser jsonWithoutProcesses, (error) ->
      test.ok error?
      test.done()

  'do not accept non-array processes': (test) ->
    new SnapshotParser jsonWithInvalidProcesses, (error) ->
      test.ok error?
      test.done()

  'do not accept empty processes': (test) ->
    new SnapshotParser jsonWithEmptyProcesses, (error) ->
      test.ok error?
      test.done()

  'have 1 snapshot': (test) ->
    new SnapshotParser validJson, (error) ->
      test.ifError(error)

      Snapshot.count (error, count) ->
        test.ifError error
        test.equals count, 1
        test.done()

  'have 2 processes': (test) ->
    new SnapshotParser validJson, (error) ->
      test.ifError(error)

      Process.count (error, count) ->
        test.ifError error
        test.equals count, 2
        test.done()

  'have 3 labels': (test) ->
    new SnapshotParser validJson, (error) ->
      test.ifError(error)

      MetricLabel.count (error, count) ->
        test.ifError error
        test.equals count, 4
        test.done()

  'have 27 metrics': (test) ->
    new SnapshotParser validJson, (error) ->
      test.ifError(error)

      Metric.count (error, count) ->
        test.ifError error
        test.equals count, 27
        test.done()
