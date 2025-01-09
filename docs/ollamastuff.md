# Ollama Stuff


## create a model including tools usage


```
FROM: qwen:0.5b

SYSTEM You are Ali G and work as a software developer.

TEMPLATE """{{ if .Messages }}
{{- if or .System .Tools }}<|im_start|>system
{{ .System }}
{{- if .Tools }}


# Tools


You are provided with function signatures within <tools></tools> XML tags. You may call one or more functions to assist with the user query. Don't make assumptions about what values to plug into functions. Here are the available tools:
<tools>{{- range .Tools }}{{ .Function }}{{- end }}</tools>

For each function call, return a JSON object with function name and arguments within <tool_call></tool_call> XML tags as follows:
<tool_call>
{"name": <function-name>, "arguments": <args-json-object>}
</tool_call>{{- end }}<|im_end|>{{- end }}
{{- range .Messages }}
{{- if eq .Role "user" }}
<|im_start|>{{ .Role }}
{{ .Content }}<|im_end|>
{{- else if eq .Role "assistant" }}
<|im_start|>{{ .Role }}
{{- if .Content }}
{{ .Content }}
{{- end }}
{{- if .ToolCalls }}
<tool_call>
{{ range .ToolCalls }}{"name": "{{ .Function.Name }}", "arguments": {{ .Function.Arguments }}}
{{ end }}</tool_call>
{{- end }}<|im_end|>
{{- else if eq .Role "tool" }}
<|im_start|>user
<tool_response>
{{ .Content }}
</tool_response><|im_end|>
{{- end }}
{{- end }}
<|im_start|>assistant
{{ else }}{{ if .System }}<|im_start|>system
{{ .System }}<|im_end|>
{{ end }}{{ if .Prompt }}<|im_start|>user
{{ .Prompt }}<|im_end|>
{{ end }}<|im_start|>assistant
{{ end }}"""

````