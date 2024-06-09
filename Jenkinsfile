pipeline {
  agent {
    dockerfile {
      filename 'Dockerfile_ubuntu_2304'
      dir 'tools/build-env'
    }
  }

  environment {
    FULL_VERSION = sh(script: "./tools/get_version.sh full", returnStdout: true).trim()
    SHORT_VERSION = sh(script: "./tools/get_version.sh", returnStdout: true).trim()
  }

  stages {
    stage('Download prerequisites') {
      steps {
        dir('ttg') {
          git url: 'https://github.com/interpretica-io/ttg.git',
              branch: 'main'
        }
      }
    }
    stage('Perform checks') {
      steps {
        sh 'env PATH=${HOME}/.cargo/bin:${PATH} cargo update -p isabelle-dm'
        sh 'env PATH=${HOME}/.cargo/bin:${PATH} env RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo fix --target wasm32-unknown-unknown && git diff --exit-code'
        sh 'env PATH=${HOME}/.cargo/bin:${PATH} env RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo fmt && git diff --exit-code'
        sh 'git tag | grep ${SHORT_VERSION}'
        sh 'cat Cargo.toml | grep ${SHORT_VERSION}'
      }
    }
    stage('Build for all platforms') {
      parallel {
        stage('Build (WASM)') {
          steps {
            sh 'env PATH=${HOME}/.cargo/bin:${PATH} ls'
            sh 'env PATH=${HOME}/.cargo/bin:${PATH} trunk --version'
            sh 'env PATH=${HOME}/.cargo/bin:${PATH} env RUSTFLAGS="--cfg=web_sys_unstable_apis" trunk build --release'
          }
        }
      }
    }
    stage('Prepare bundle') {
      stages {
        stage('Prepare artifacts (branch)') {
          steps {
            sh 'mkdir -p build && (rm -rf build/* || true)'
            /* Create branch-build-linux and doc-branch-build */
            sh './tools/release.sh --out build/sample-ui-${BRANCH_NAME}-${BUILD_NUMBER}-wasm.tar.xz'
            /* Copy branch-build-linux to branch-latest-linux */
            sh 'cp build/sample-ui-${BRANCH_NAME}-${BUILD_NUMBER}-wasm.tar.xz build/sample-ui-${BRANCH_NAME}-latest-wasm.tar.xz'
          }
        }
        stage('Prepare artifacts (versioned)') {
          when {
            expression {
              BRANCH_NAME == 'main'
            }
          }
          steps {
          /* Create versioned artifacts */
            sh 'mkdir -p build/versioned_artifacts'

            /* Copy branch-latest-linux to fullver-linux */
            sh 'cp build/sample-ui-${BRANCH_NAME}-latest-wasm.tar.xz build/versioned_artifacts/sample-ui-${FULL_VERSION}-wasm.tar.xz'
          }
        }
      }
    }
    stage('Publish artifacts') {
      parallel {
        stage('Publish artifacts (branch)') {
          steps {
            ftpPublisher alwaysPublishFromMaster: true,
                         continueOnError: false,
                         failOnError: false,
                         masterNodeName: '',
                         paramPublish: null,
                         publishers: [
                          [
                            configName: 'sample-ui releases',
                            transfers:
                              [[
                                asciiMode: false,
                                cleanRemote: false,
                                excludes: '',
                                flatten: false,
                                makeEmptyDirs: false,
                                noDefaultExcludes: false,
                                patternSeparator: '[, ]+',
                                remoteDirectory: 'branches/${BRANCH_NAME}-${BUILD_NUMBER}',
                                remoteDirectorySDF: false,
                                removePrefix: 'build',
                                sourceFiles: 'build/sample-ui-*${BRANCH_NAME}-${BUILD_NUMBER}*.tar.xz'
                              ]],
                            usePromotionTimestamp: false,
                            useWorkspaceInPromotion: false,
                            verbose: true
                          ]
                        ]
            ftpPublisher alwaysPublishFromMaster: true,
                         continueOnError: false,
                         failOnError: false,
                         masterNodeName: '',
                         paramPublish: null,
                         publishers: [
                          [
                            configName: 'sample-ui releases',
                            transfers:
                              [[
                                asciiMode: false,
                                cleanRemote: false,
                                excludes: '',
                                flatten: false,
                                makeEmptyDirs: false,
                                noDefaultExcludes: false,
                                patternSeparator: '[, ]+',
                                remoteDirectory: 'branches/${BRANCH_NAME}',
                                remoteDirectorySDF: false,
                                removePrefix: 'build',
                                sourceFiles: 'build/sample-ui-*${BRANCH_NAME}-latest*.tar.xz'
                              ]],
                            usePromotionTimestamp: false,
                            useWorkspaceInPromotion: false,
                            verbose: true
                          ]
                        ]
          }
        }
        stage('Publish artifacts (versioned)') {
          when {
            expression {
              BRANCH_NAME == 'main'
            }
          }
          steps {
            ftpPublisher alwaysPublishFromMaster: true,
                         continueOnError: false,
                         failOnError: false,
                         masterNodeName: '',
                         paramPublish: null,
                         publishers: [
                          [
                            configName: 'sample-ui releases',
                            transfers:
                              [[
                                asciiMode: false,
                                cleanRemote: false,
                                excludes: '',
                                flatten: false,
                                makeEmptyDirs: false,
                                noDefaultExcludes: false,
                                patternSeparator: '[, ]+',
                                remoteDirectory: "${FULL_VERSION}",
                                remoteDirectorySDF: false,
                                removePrefix: 'build/versioned_artifacts',
                                sourceFiles: 'build/versioned_artifacts/sample-ui-*.tar.xz'
                              ]],
                            usePromotionTimestamp: false,
                            useWorkspaceInPromotion: false,
                            verbose: true
                          ]
                        ]
          }
        }
        stage('Archive artifacts for Jenkins') {
          steps {
            archiveArtifacts artifacts: 'build/sample-ui-*.tar.xz'
          }
        }
      }
    }
  }
  post {
    success {
      sh './ttg/ttg_send_notification --env --ignore-bad -- "${JOB_NAME}/${BUILD_NUMBER}: PASSED"'
    }
    failure {
      sh './ttg/ttg_send_notification --env --ignore-bad -- "${JOB_NAME}/${BUILD_NUMBER}: FAILED. See details in ${BUILD_URL}"'
    }
  }
}
