// Launch one bundled editor through LaunchServices with an explicit argument
// list and environment, then print the launched process id.
//
// Apple's cooperative activation (macOS 14+) makes activation a request the
// system grants from context; a process exec'd straight from a shell has no such
// context. `NSWorkspace.OpenConfiguration` is the supported entry: it can pass
// arguments and an environment, request a new instance and ask for activation.
// The caller verifies the returned pid's executable before trusting it.
import AppKit
import Foundation

func fail(_ message: String) -> Never {
  FileHandle.standardError.write(("ime-launch: " + message + "\n").data(using: .utf8)!)
  exit(2)
}

let arguments = CommandLine.arguments
guard arguments.count >= 3 else {
  fail("usage: ime-launch <bundle-path> <json-spec>")
}
let bundlePath = arguments[1]
guard let spec = try? String(contentsOfFile: arguments[2], encoding: .utf8),
      let data = spec.data(using: .utf8),
      let document = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
      let argv = document["arguments"] as? [String] else {
  fail("unreadable launch spec")
}
let environment = (document["environment"] as? [String: String]) ?? [:]
let bundle = URL(fileURLWithPath: bundlePath)
guard bundle.pathExtension == "app", FileManager.default.fileExists(atPath: bundle.path) else {
  fail("exact bundle path required: " + bundlePath)
}

let configuration = NSWorkspace.OpenConfiguration()
configuration.arguments = argv
configuration.environment = environment
configuration.createsNewApplicationInstance = true
configuration.activates = true
configuration.addsToRecentItems = false

let done = DispatchSemaphore(value: 0)
var status: Int32 = 1
NSWorkspace.shared.openApplication(at: bundle, configuration: configuration) { app, error in
  if let error = error {
    FileHandle.standardError.write(("ime-launch: " + error.localizedDescription + "\n").data(using: .utf8)!)
  } else if let app = app {
    print(app.processIdentifier)
    status = 0
  } else {
    FileHandle.standardError.write("ime-launch: no application returned\n".data(using: .utf8)!)
  }
  done.signal()
}
if done.wait(timeout: .now() + 20) == .timedOut { fail("LaunchServices did not answer") }
exit(status)
