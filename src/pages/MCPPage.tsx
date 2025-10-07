import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "@/components/ui/accordion";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { HammerIcon, PlusIcon, SaveIcon } from "lucide-react";
import { useGlobalMcpServers, useUpdateGlobalMcpServer, type McpServer } from "@/lib/query";
import { toast } from "sonner";

export function MCPPage() {
  const { data: mcpServers, isLoading, error } = useGlobalMcpServers();
  const updateMcpServer = useUpdateGlobalMcpServer();
  const [serverConfigs, setServerConfigs] = useState<Record<string, string>>({});
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const handleConfigChange = (serverName: string, configText: string) => {
    setServerConfigs(prev => ({
      ...prev,
      [serverName]: configText
    }));
  };

  const handleSaveConfig = (serverName: string) => {
    const configText = serverConfigs[serverName];
    if (!configText) return;

    try {
      const configObject = JSON.parse(configText);
      updateMcpServer.mutate({
        serverName,
        serverConfig: configObject
      });
    } catch (error) {
      toast.error(`Invalid JSON configuration for ${serverName}`);
    }
  };

  
  const formatConfigForDisplay = (server: McpServer): string => {
    return JSON.stringify(server, null, 2);
  };

  if (isLoading) {
    return (
      <div className="p-4">
        <div className="text-center">Loading MCP servers...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4">
        <div className="text-center text-red-500">
          Failed to load MCP servers: {error.message}
        </div>
      </div>
    );
  }

  const serverEntries = Object.entries(mcpServers || {});

  return (
    <div className="">
      <div className="flex items-center p-3 border-b px-3 justify-between sticky top-0 bg-background z-10" data-tauri-drag-region>
        <div data-tauri-drag-region>
          <h3 className="font-bold" data-tauri-drag-region>MCP</h3>
          <p className="text-sm text-muted-foreground" data-tauri-drag-region>
            Claude Code 全局 MCP 服务配置
          </p>
        </div>
        <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
          <DialogTrigger asChild>
            <Button variant="ghost" className="text-muted-foreground" size="sm">
              <PlusIcon size={14} />
              添加 MCP 服务
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>添加新的 MCP 服务</DialogTitle>
            </DialogHeader>
            <div className="py-4">
              <p className="text-muted-foreground">添加新的 MCP 服务功能正在开发中...</p>
            </div>
            <div className="flex justify-end">
              <Button
                variant="outline"
                onClick={() => setIsDialogOpen(false)}
              >
                关闭
              </Button>
            </div>
          </DialogContent>
        </Dialog>
      </div>
      <div className="">
        {serverEntries.length === 0 ? (
          <div className="text-center text-muted-foreground py-8">
            No MCP servers configured. Click "添加 MCP 服务" to add one.
          </div>
        ) : (
          <Accordion type="multiple" className="">
            {serverEntries.map(([serverName, serverConfig]) => (
              <AccordionItem key={serverName} value={serverName} className="bg-zinc-50">
                <AccordionTrigger className="hover:no-underline px-4 py-2 bg-zinc-50 hover:bg-zinc-100  duration-150">
                  <div className="flex items-center gap-2">
                    <HammerIcon size={12} />
                    <span className="font-medium">{serverName}</span>
                  </div>
                </AccordionTrigger>
                <AccordionContent className="pb-3">
                  <div className="px-3 pt-3 space-y-3">
                    <textarea
                      value={serverConfigs[serverName] || formatConfigForDisplay(serverConfig)}
                      onChange={(e) => handleConfigChange(serverName, e.target.value)}
                      placeholder="Enter MCP server configuration as JSON"
                      className="min-h-[100px] w-full p-2 text-md outline-none resize-none bg-white rounded-lg"
                      spellCheck={false}
                    />
                    <div className="flex  bg-zinc-50">
                      <Button
                        variant="outline"
                        onClick={() => handleSaveConfig(serverName)}
                        disabled={updateMcpServer.isPending}
                        size="sm"
                      >
                        <SaveIcon size={14} className="" />
                        {updateMcpServer.isPending ? "Saving..." : "Save"}
                      </Button>
                    </div>
                  </div>
                </AccordionContent>
              </AccordionItem>
            ))}
          </Accordion>
        )}
      </div>
    </div>
  );
}