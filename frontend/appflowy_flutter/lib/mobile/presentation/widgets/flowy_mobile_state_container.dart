import 'dart:io';

import 'package:appflowy/generated/locale_keys.g.dart';
import 'package:appflowy_editor/appflowy_editor.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:package_info_plus/package_info_plus.dart';

enum _FlowyMobileStateContainerType {
  info,
  error,
}

/// Used to display info(like empty state) or error state
/// error state has two buttons to report issue with error message or reach out on discord
class FlowyMobileStateContainer extends StatelessWidget {
  const FlowyMobileStateContainer.error({
    this.emoji,
    required this.title,
    this.description,
    required this.errorMsg,
    super.key,
  }) : _stateType = _FlowyMobileStateContainerType.error;

  const FlowyMobileStateContainer.info({
    this.emoji,
    required this.title,
    this.description,
    super.key,
  })  : errorMsg = null,
        _stateType = _FlowyMobileStateContainerType.info;

  final String? emoji;
  final String title;
  final String? description;
  final String? errorMsg;
  final _FlowyMobileStateContainerType _stateType;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return SizedBox.expand(
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 8, horizontal: 32),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Text(
              emoji ?? '',
              style: const TextStyle(fontSize: 40),
            ),
            const SizedBox(height: 8),
            Text(
              title,
              style: theme.textTheme.labelLarge,
            ),
            const SizedBox(height: 4),
            Text(
              description ?? '',
              style: theme.textTheme.bodyMedium?.copyWith(
                color: theme.hintColor,
              ),
            ),
            if (_stateType == _FlowyMobileStateContainerType.error) ...[
              const SizedBox(height: 8),
              FutureBuilder(
                future: PackageInfo.fromPlatform(),
                builder: (context, snapshot) {
                  return Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      OutlinedButton(
                        onPressed: () {
                          final String? version = snapshot.data?.version;
                          final String os = Platform.operatingSystem;
                          safeLaunchUrl(
                            'https://github.com/AppFlowy-IO/AppFlowy/issues/new?assignees=&labels=&projects=&template=bug_report.yaml&title=[Bug]%20Mobile:%20&version=$version&os=$os&context=Error%20log:%20$errorMsg',
                          );
                        },
                        child: Text(
                          LocaleKeys.workspace_errorActions_reportIssue.tr(),
                        ),
                      ),
                      OutlinedButton(
                        onPressed: () =>
                            safeLaunchUrl('https://discord.gg/JucBXeU2FE'),
                        child: Text(
                          LocaleKeys.workspace_errorActions_reachOut.tr(),
                        ),
                      ),
                    ],
                  );
                },
              )
            ]
          ],
        ),
      ),
    );
  }
}
