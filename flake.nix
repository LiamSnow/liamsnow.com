{
  description = "liamsnow.com";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        pname = cargoToml.package.name;
        version = cargoToml.package.version;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          inherit pname version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildType = "release";
          nativeBuildInputs = [ pkgs.makeWrapper ];
          
          postInstall = ''
            mkdir -p $out/share/${pname}
            
            for dir in static blog projects; do
              if [ ! -d "$dir" ]; then
                echo "Error: Required directory '$dir' not found" >&2
                exit 1
              fi
              cp -r $dir $out/share/${pname}/
            done
          '';
        };
      }
    ) // {
      nixosModules.default = { config, lib, pkgs, ... }: 
        with lib;
        let
          cfg = config.services.liamsnow-com;
        in
        {
          options.services.liamsnow-com = {
            enable = mkEnableOption "liamsnow.com";
            
            package = mkOption {
              type = types.package;
              default = self.packages.${pkgs.system}.default;
            };
            
            port = mkOption {
              type = types.port;
              default = 8080;
              description = "Port to listen on";
            };
            
            address = mkOption {
              type = types.str;
              default = "127.0.0.1";
              description = "Address to bind to";
            };
            
            workingDir = mkOption {
              type = types.str;
              default = "${cfg.package}/share/liamsnow-com";
              description = "Working directory containing static, blog, and projects directories";
            };
            
            openFirewall = mkOption {
              type = types.bool;
              default = false;
              description = "Whether to open the firewall for this service";
            };
          };
          
          config = mkIf cfg.enable {
            networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [ cfg.port ];
            
            systemd.services.liamsnow-com = {
              description = "liamsnow.com";
              wantedBy = [ "multi-user.target" ];
              after = [ "network.target" ];
              
              serviceConfig = {
                ExecStart = "${cfg.package}/bin/liamsnow-com --port ${toString cfg.port} --address ${cfg.address} --working-director ${cfg.workingDir}";
                Restart = "always";
                RestartSec = 5;
                Type = "simple";
                
                DynamicUser = true;
                PrivateTmp = true;
                ProtectSystem = "strict";
                ProtectHome = true;
                NoNewPrivileges = true;
                
                RestrictAddressFamilies = "AF_INET AF_INET6";
                CapabilityBoundingSet = "";
                LockPersonality = true;
                MemoryDenyWriteExecute = true;
                PrivateDevices = true;
                ProtectClock = true;
                ProtectControlGroups = true;
                ProtectKernelLogs = true;
                ProtectKernelModules = true;
                ProtectKernelTunables = true;
                RestrictNamespaces = true;
                RestrictRealtime = true;
                RestrictSUIDSGID = true;
                SystemCallArchitectures = "native";
                SystemCallFilter = "@system-service";
              };
            };
          };
        };
    };
}
