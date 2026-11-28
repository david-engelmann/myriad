# PowerShell completion for myriad
Register-ArgumentCompleter -Native -CommandName myriad -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)
  $subcommands = @('map', 'check', 'claims', 'census', 'atlas', 'index', 'changelog', 'lineage', 'returns', 'standing', 'absence', 'arrivals', 'departures', 'matched', 'runs', 'presence', 'standings', 'company', 'sets', 'rosters', 'fellows', '-h', '--help')
  $elements = $commandAst.CommandElements
  $argIndex = 0
  for ($i = 0; $i -lt $elements.Count; $i++) {
    $ext = $elements[$i].Extent
    if ($cursorPosition -le $ext.EndOffset) {
      $argIndex = $i
      break
    }
    $argIndex = $i + 1
  }
  if ($argIndex -eq 1) {
    $subcommands | Where-Object { $_.StartsWith($wordToComplete, [System.StringComparison]::Ordinal) } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
    return
  }
  if ($argIndex -ge 2 -and $wordToComplete.StartsWith('-')) {
    @('-h', '--help') | Where-Object { $_.StartsWith($wordToComplete, [System.StringComparison]::Ordinal) } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterName', $_)
    }
    return
  }
}
