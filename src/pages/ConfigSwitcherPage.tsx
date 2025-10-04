import { useNavigate } from "react-router-dom";
import { useStores, useCurrentStore, useSetCurrentStore } from "../lib/query";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { StoreManager } from "../components/StoreManager";
import { cn } from "@/lib/utils";

export function ConfigSwitcherPage() {
  const navigate = useNavigate();
  const { data: stores } = useStores();
  const { data: currentStore } = useCurrentStore();
  const setCurrentStoreMutation = useSetCurrentStore();

  const handleSelectStore = (storeName: string) => {
    setCurrentStoreMutation.mutate(storeName);
  };

  const handleEditConfig = () => {
    navigate("/config/editor");
  };

  const handleCreateStore = () => {
    navigate("/stores/new");
  };

  const handleEditStore = (storeName: string) => {
    navigate(`/stores/${storeName}/edit`);
  };

  return (
    <div className="p-3">
      <section>
        <ConfigStores />
      </section>
    </div>
  );
}

function ConfigStores() {
  const { data: stores } = useStores();
  return (
    <div className="grid grid-cols-4 gap-3">
      {stores.map((store) => {
        const isCurrentStore = store.using
        return (
          <div key={store.name} className={cn("border rounded-xl p-3 h-[100px]", {
            "bg-primary/10 border-primary border-2": isCurrentStore,
          })}>
            {store.name}
          </div>
        )
      })}
    </div>
  )
}