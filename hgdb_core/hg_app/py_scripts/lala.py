from data_loader import DataLoader

data = DataLoader()

edge = data.get_statics_root() / "statics" / "interactive.html"
print(edge)

at = data.get_resources_root() / "resources" / "Atoms.json"
print(at)