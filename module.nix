{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.odilf-com;
  odilf-com-pkg = pkgs.callPackage ./default.nix { };

  # Taken from immich
  commonServiceConfig = {
    Type = "simple";

    # Hardening
    CapabilityBoundingSet = "";
    NoNewPrivileges = true;
    PrivateUsers = true;
    PrivateTmp = true;
    PrivateDevices = true;
    PrivateMounts = true;
    ProtectClock = true;
    ProtectControlGroups = true;
    ProtectHome = true;
    ProtectHostname = true;
    ProtectKernelLogs = true;
    ProtectKernelModules = true;
    ProtectKernelTunables = true;
    RestrictAddressFamilies = [
      "AF_INET"
      "AF_INET6"
      "AF_UNIX"
    ];
    RestrictNamespaces = true;
    RestrictRealtime = true;
    RestrictSUIDSGID = true;
  };
in
{
  options.services.odilf-com = {
    enable = lib.mkEnableOption "odilf.com";
    port = lib.mkOption {
      type = lib.types.port;
      default = 6428;
      description = "Port to listen on";
    };

    addr = lib.mkOption {
      type = lib.types.str;
      default = "0.0.0.0";
      description = "Address to listen on";
    };

    blog-path = lib.mkOption {
      type = lib.types.path;
      description = "Path to blog's markdown files";
    };
  };

  config.systemd.services.odilf-com = lib.mkIf cfg.enable {
    description = "odilf.com personal site";
    after = [ "network.target" ];
    wantedBy = [ "multi-user.target" ];

    serviceConfig = commonServiceConfig // {
      ExecStart = "${odilf-com-pkg}/bin/odilf-com";
      StateDirectory = "odilf-com";
      SyslogIdentifier = "odilf-com";
      RuntimeDirectory = "odilf-com";
      User = "root";
      Group = "root";
    };

    environment = {
      "ODILF_BLOG_PATH" = cfg.odilf-com.blog-path;
      "LEPTOS_SITE_ADDR" = "${cfg.addr}:${cfg.port}"
  };
}
