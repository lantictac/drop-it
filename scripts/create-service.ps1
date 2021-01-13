if ($args.Count -ne 1) {
    throw "create-service requires at least 1 argument for the binary path"
}

$params = @{
    Name           = "DropIt"
    BinaryPathName = "'$($args[0])'"
    DisplayName    = "Drop It"
    StartupType    = "AutomaticDelayedStart"
    Description    = "Remote shutdown service"
}
New-Service @params
