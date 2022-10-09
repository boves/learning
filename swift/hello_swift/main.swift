func foo() {
    let name = CommandLine.arguments[1]
    print("Hello \(name)! Let's go to the \(Event.gym.rawValue)!")
}

foo()
